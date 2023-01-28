import graphene
from graphene_django import DjangoObjectType
from polls.models import Student

class StudentType(DjangoObjectType):
    class Meta:
        model = Student
        fields = ("id", "firstname", "lastname", "email")

class Query(graphene.ObjectType):
    all_students = graphene.List(StudentType)
    category_by_id = graphene.Field(StudentType, sex=graphene.String(required=True))
    category_by_sex = graphene.Field(StudentType, sex=graphene.String(required=True))
    
    def resolve_all_students(root, info):
        print(root)
        return Student.objects.all()
    
    def resolve_category_by_id(root, info, id):
        return Student.objects.get(id=id)

    def resolve_student_by_sex(root, info, sex):
        try:
            return Student.objects.get(sex=sex)
        except Student.DoesNotExist:
            return None
        
class StudentMutation(graphene.Mutation):
    class Arguments:
        # The input arguments for this mutation
        email = graphene.String(required=True)
        id = graphene.ID()

    # The class attributes define the response of the mutation
    latest_student = graphene.Field(StudentType)

    @classmethod
    def mutate(cls, root, info, email, id):
        student = Student.objects.get(id=id)
        student.email = email
        student.save()
        # Notice we return an instance of this mutation
        return StudentMutation(latest_student=student)
    
class Mutation(graphene.ObjectType):
    update_email = StudentMutation.Field()

schema = graphene.Schema(query=Query, mutation=Mutation)
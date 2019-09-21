import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper
Date date = new Date()

String datevalue = date.toString()

datevalue = datevalue.replace(' ', '')

datevalue = datevalue.replace(':', '')

println(datevalue)

//GlobalVariable.EmpName = datevalue
EmpName=datevalue

//println(GlobalVariable.EmpName)

response = WS.sendRequest(findTestObject('DummyEmp/CreateEmp', ['EmpName':EmpName,'salary':'123','age':'12']))

 
println response.getResponseBodyContent()
WS.verifyResponseStatusCode(response, 200)


String jsonString = response.getResponseBodyContent()
JsonSlurper slurper = new JsonSlurper()
Map parsedJson = slurper.parseText(jsonString)

String idValue = parsedJson.get("id")

EmpID = idValue

 

EmpName = datevalue
response = WS.sendRequest(findTestObject('DummyEmp/Update_Employee', ['EmpID':EmpID,'EmpName':EmpName,'Salary':'1234','Age':'24']))
 

println response.getResponseBodyContent()


response = WS.sendRequest(findTestObject('DummyEmp/Delete_Emp', ['EmpID':EmpID]))
println response.getResponseBodyContent()